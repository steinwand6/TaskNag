const Database = require('better-sqlite3');

const dbPath = 'C:\\Users\\stone\\AppData\\Roaming\\com.tasknag.app\\tasknag.db';

try {
    const db = new Database(dbPath);
    
    console.log('=== Checking actual database schema ===');
    
    // Check tasks table schema
    const schema = db.prepare("PRAGMA table_info(tasks)").all();
    console.log('\nTasks table columns:');
    schema.forEach(col => {
        console.log(`  ${col.name}: ${col.type} (${col.notnull ? 'NOT NULL' : 'NULL'}) ${col.pk ? '(PRIMARY KEY)' : ''}`);
    });
    
    // Check if browser_actions column exists
    const browserActionsColumn = schema.find(col => col.name === 'browser_actions');
    console.log(`\nbrowser_actions column exists: ${browserActionsColumn ? 'YES' : 'NO'}`);
    
    if (browserActionsColumn) {
        console.log(`  Type: ${browserActionsColumn.type}`);
        console.log(`  Nullable: ${!browserActionsColumn.notnull ? 'YES' : 'NO'}`);
    }
    
    // Check migration history
    console.log('\n=== Checking migration history ===');
    try {
        const migrations = db.prepare("SELECT * FROM _sqlx_migrations ORDER BY version").all();
        console.log('Applied migrations:');
        migrations.forEach(m => {
            console.log(`  Version: ${m.version}, Success: ${m.success}, Checksum: ${m.checksum}`);
        });
    } catch (e) {
        console.log('No migration table found or migration table is empty');
    }
    
    db.close();
} catch (error) {
    console.error('Error checking database:', error.message);
}