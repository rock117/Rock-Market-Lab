#!/usr/bin/env python3
"""
Database DDL Export Script

This script exports the DDL (Data Definition Language) statements for all tables
in the investment_research PostgreSQL database. It generates CREATE TABLE statements
without any data.

Usage:
    python export_db_ddl.py [--output-file output.sql] [--host localhost] [--port 5432] [--user postgres] [--password 123456] [--database investment_research]

Requirements:
    pip install psycopg2-binary
"""

import argparse
import sys
import os
from datetime import datetime
try:
    import psycopg2
    from psycopg2 import sql
except ImportError:
    print("Error: psycopg2 is required. Install it with: pip install psycopg2-binary")
    sys.exit(1)


def get_table_ddl(cursor, table_name, schema='public'):
    """
    Generate CREATE TABLE DDL for a specific table
    """
    # Get table columns information
    cursor.execute("""
        SELECT 
            column_name,
            data_type,
            character_maximum_length,
            numeric_precision,
            numeric_scale,
            is_nullable,
            column_default
        FROM information_schema.columns 
        WHERE table_schema = %s AND table_name = %s
        ORDER BY ordinal_position;
    """, (schema, table_name))
    
    columns = cursor.fetchall()
    if not columns:
        return None
    
    # Get primary key information
    cursor.execute("""
        SELECT column_name
        FROM information_schema.table_constraints tc
        JOIN information_schema.key_column_usage kcu 
            ON tc.constraint_name = kcu.constraint_name
            AND tc.table_schema = kcu.table_schema
        WHERE tc.constraint_type = 'PRIMARY KEY'
            AND tc.table_schema = %s 
            AND tc.table_name = %s
        ORDER BY kcu.ordinal_position;
    """, (schema, table_name))
    
    primary_keys = [row[0] for row in cursor.fetchall()]
    
    # Get foreign key information
    cursor.execute("""
        SELECT
            kcu.column_name,
            ccu.table_name AS foreign_table_name,
            ccu.column_name AS foreign_column_name,
            tc.constraint_name
        FROM information_schema.table_constraints AS tc
        JOIN information_schema.key_column_usage AS kcu
            ON tc.constraint_name = kcu.constraint_name
            AND tc.table_schema = kcu.table_schema
        JOIN information_schema.constraint_column_usage AS ccu
            ON ccu.constraint_name = tc.constraint_name
            AND ccu.table_schema = tc.table_schema
        WHERE tc.constraint_type = 'FOREIGN KEY'
            AND tc.table_schema = %s
            AND tc.table_name = %s;
    """, (schema, table_name))
    
    foreign_keys = cursor.fetchall()
    
    # Build CREATE TABLE statement
    ddl_lines = [f"CREATE TABLE {table_name} ("]
    
    column_definitions = []
    for col in columns:
        col_name, data_type, char_max_len, num_precision, num_scale, is_nullable, col_default = col
        
        # Build column definition
        col_def = f"    {col_name}"
        
        # Add data type
        if data_type == 'character varying':
            if char_max_len:
                col_def += f" VARCHAR({char_max_len})"
            else:
                col_def += " VARCHAR"
        elif data_type == 'character':
            col_def += f" CHAR({char_max_len})" if char_max_len else " CHAR"
        elif data_type == 'numeric':
            if num_precision and num_scale:
                col_def += f" NUMERIC({num_precision},{num_scale})"
            elif num_precision:
                col_def += f" NUMERIC({num_precision})"
            else:
                col_def += " NUMERIC"
        elif data_type == 'integer':
            col_def += " INTEGER"
        elif data_type == 'bigint':
            col_def += " BIGINT"
        elif data_type == 'smallint':
            col_def += " SMALLINT"
        elif data_type == 'boolean':
            col_def += " BOOLEAN"
        elif data_type == 'date':
            col_def += " DATE"
        elif data_type == 'timestamp without time zone':
            col_def += " TIMESTAMP"
        elif data_type == 'timestamp with time zone':
            col_def += " TIMESTAMPTZ"
        elif data_type == 'text':
            col_def += " TEXT"
        elif data_type == 'json':
            col_def += " JSON"
        elif data_type == 'jsonb':
            col_def += " JSONB"
        else:
            col_def += f" {data_type.upper()}"
        
        # Add NOT NULL constraint
        if is_nullable == 'NO':
            col_def += " NOT NULL"
        
        # Add default value
        if col_default:
            col_def += f" DEFAULT {col_default}"
        
        column_definitions.append(col_def)
    
    ddl_lines.extend(column_definitions)
    
    # Add primary key constraint
    if primary_keys:
        pk_constraint = f"    PRIMARY KEY ({', '.join(primary_keys)})"
        ddl_lines.append(pk_constraint)
    
    # Add foreign key constraints
    for fk in foreign_keys:
        col_name, foreign_table, foreign_col, constraint_name = fk
        fk_constraint = f"    CONSTRAINT {constraint_name} FOREIGN KEY ({col_name}) REFERENCES {foreign_table}({foreign_col})"
        ddl_lines.append(fk_constraint)
    
    # Join all parts
    ddl = ",\n".join(ddl_lines[:-len(foreign_keys) if foreign_keys else 0])
    if foreign_keys:
        ddl += ",\n" + ",\n".join(ddl_lines[-len(foreign_keys):])
    ddl += "\n);"
    
    return ddl


def get_indexes_ddl(cursor, table_name, schema='public'):
    """
    Generate CREATE INDEX statements for a table
    """
    cursor.execute("""
        SELECT
            indexname,
            indexdef
        FROM pg_indexes
        WHERE schemaname = %s 
            AND tablename = %s
            AND indexname NOT LIKE '%_pkey';
    """, (schema, table_name))
    
    indexes = cursor.fetchall()
    index_ddls = []
    
    for index_name, index_def in indexes:
        index_ddls.append(f"{index_def};")
    
    return index_ddls


def export_database_ddl(host, port, database, user, password, output_file=None):
    """
    Export DDL for all tables in the database
    """
    try:
        # Connect to database
        conn = psycopg2.connect(
            host=host,
            port=port,
            database=database,
            user=user,
            password=password
        )
        cursor = conn.cursor()
        
        # Get all tables
        cursor.execute("""
            SELECT table_name 
            FROM information_schema.tables 
            WHERE table_schema = 'public' 
                AND table_type = 'BASE TABLE'
            ORDER BY table_name;
        """)
        
        tables = [row[0] for row in cursor.fetchall()]
        
        if not tables:
            print("No tables found in the database.")
            return
        
        # Generate DDL
        ddl_content = []
        ddl_content.append(f"-- Database DDL Export")
        ddl_content.append(f"-- Database: {database}")
        ddl_content.append(f"-- Export Date: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
        ddl_content.append(f"-- Total Tables: {len(tables)}")
        ddl_content.append("")
        
        for table_name in tables:
            print(f"Exporting table: {table_name}")
            
            ddl_content.append(f"-- Table: {table_name}")
            ddl_content.append("-" * 50)
            
            # Get table DDL
            table_ddl = get_table_ddl(cursor, table_name)
            if table_ddl:
                ddl_content.append(table_ddl)
            
            # Get indexes DDL
            indexes_ddl = get_indexes_ddl(cursor, table_name)
            if indexes_ddl:
                ddl_content.append("")
                ddl_content.append(f"-- Indexes for {table_name}")
                ddl_content.extend(indexes_ddl)
            
            ddl_content.append("")
        
        # Write to file or stdout
        output_text = "\n".join(ddl_content)
        
        if output_file:
            with open(output_file, 'w', encoding='utf-8') as f:
                f.write(output_text)
            print(f"DDL exported to: {output_file}")
        else:
            print(output_text)
        
        cursor.close()
        conn.close()
        
        print(f"Successfully exported DDL for {len(tables)} tables.")
        
    except psycopg2.Error as e:
        print(f"Database error: {e}")
        sys.exit(1)
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)


def main():
    parser = argparse.ArgumentParser(
        description="Export PostgreSQL database DDL (table structures only)",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
    # Export to file
    python export_db_ddl.py --output-file schema.sql
    
    # Export to stdout with custom connection
    python export_db_ddl.py --host localhost --port 5432 --user postgres --password mypass --database mydb
    
    # Export with default settings (from config)
    python export_db_ddl.py
        """
    )
    
    parser.add_argument('--host', default='127.0.0.1', help='Database host (default: 127.0.0.1)')
    parser.add_argument('--port', type=int, default=5432, help='Database port (default: 5432)')
    parser.add_argument('--database', default='investment_research', help='Database name (default: investment_research)')
    parser.add_argument('--user', default='postgres', help='Database user (default: postgres)')
    parser.add_argument('--password', default='123456', help='Database password (default: 123456)')
    parser.add_argument('--output-file', '-o', help='Output file path (default: stdout)')
    parser.add_argument('--verbose', '-v', action='store_true', help='Verbose output')
    
    args = parser.parse_args()
    
    if args.verbose:
        print(f"Connecting to {args.host}:{args.port}/{args.database} as {args.user}")
    
    export_database_ddl(
        host=args.host,
        port=args.port,
        database=args.database,
        user=args.user,
        password=args.password,
        output_file=args.output_file
    )


# if __name__ == '__main__':
#     main()
main()