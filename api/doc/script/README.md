# Database DDL Export Script

This directory contains scripts for database operations.

## export_db_ddl.py

A Python script to export PostgreSQL database DDL (Data Definition Language) statements for all tables in the `investment_research` database.

### Features

- Exports CREATE TABLE statements with complete column definitions
- Includes primary key constraints
- Includes foreign key constraints  
- Includes index definitions
- Supports custom connection parameters
- Can output to file or stdout

### Requirements

```bash
pip install psycopg2-binary
```

### Usage

#### Basic usage (uses default connection settings from config):
```bash
python export_db_ddl.py
```

#### Export to file:
```bash
python export_db_ddl.py --output-file schema.sql
```

#### Custom connection parameters:
```bash
python export_db_ddl.py --host localhost --port 5432 --user postgres --password mypass --database investment_research --output-file schema.sql
```

#### Show help:
```bash
python export_db_ddl.py --help
```

### Default Connection Settings

The script uses the following default settings (matching your config):
- Host: `127.0.0.1`
- Port: `5432`
- Database: `investment_research`
- User: `postgres`
- Password: `123456`

### Output Format

The generated DDL includes:
- Header with export metadata
- CREATE TABLE statements for each table
- Column definitions with data types, constraints, and defaults
- Primary key constraints
- Foreign key constraints
- Index definitions

### Example Output

```sql
-- Database DDL Export
-- Database: investment_research
-- Export Date: 2024-07-24 23:11:14
-- Total Tables: 25

-- Table: stock_daily
--------------------------------------------------
CREATE TABLE stock_daily (
    ts_code VARCHAR(20) NOT NULL,
    trade_date VARCHAR(20) NOT NULL,
    open NUMERIC,
    high NUMERIC,
    low NUMERIC,
    close NUMERIC,
    pre_close NUMERIC,
    change NUMERIC,
    pct_chg NUMERIC,
    vol NUMERIC,
    amount NUMERIC,
    PRIMARY KEY (ts_code, trade_date)
);
```
