#!/usr/bin/env python3

import subprocess
import sys
import os
import argparse
from datetime import datetime
from pathlib import Path

def export_schema(host, port, username, database, output_file, password=None):
    """
    Export PostgreSQL database schema using pg_dump
    
    Args:
        host (str): Database host
        port (str): Database port
        username (str): Database username
        database (str): Database name
        output_file (str): Output file path
        password (str, optional): Database password
    
    Returns:
        bool: True if export successful, False otherwise
    """
    
    # Build pg_dump command
    cmd = [
        'pg_dump',
        '-h', host,
        '-p', str(port),
        '-U', username,
        '-d', database,
        '--schema-only',  # Export only schema, no data
        '--no-owner',     # Don't output commands to set ownership
        '--no-privileges', # Don't output commands to set privileges
        '--verbose',      # Verbose output
        '-f', f'{output_file}'
    ]
    
    try:
        print(f"Exporting schema from database '{database}' to '{output_file}'...")
        print(f"Command: {' '.join(cmd)}")
        os.chdir("C:/rock/coding/code/my/rust/Rock-Market-Lab/api/doc/script/db/sql")
        os.system(" ".join(cmd))
        return True
    except Exception as e:
        print(f"Unexpected error: {e}")
        return False


def main():
    """Main function to handle command line arguments and execute export"""
    host = '127.0.0.1'
    port = 15432
    username = 'postgres'
    database = 'investment_research'
    password = '123456'
    
    # Create output directory if it doesn't exist
    output_path = 'C:/rock/coding/code/my/rust/Rock-Market-Lab/api/doc/script/db/sql/investment_research.sql'
    
    # Display connection info
    print("PostgreSQL Schema Export")
    print("=" * 40)
    print(f"Host: {host}")
    print(f"Port: {port}")
    print(f"Username: {username}")
    print(f"Database: {database}")
    print(f"Output: {output_path}")
    print("=" * 40)
    
    # Export schema
    success = export_schema(
        host=host,
        port=port,
        username=username,
        database=database,
        output_file=output_path,
        password=password
    )
    
    if success:
        print("\n✅ Schema export completed successfully!")
        sys.exit(0)
    else:
        print("\n❌ Schema export failed!")
        sys.exit(1)

main()
