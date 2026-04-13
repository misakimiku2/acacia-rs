DEFINE TABLE IF NOT EXISTS player_attr SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS pid   ON player_attr TYPE int;
DEFINE FIELD IF NOT EXISTS key   ON player_attr TYPE int;
DEFINE FIELD IF NOT EXISTS value ON player_attr TYPE int DEFAULT 0;

DEFINE INDEX IF NOT EXISTS idx_attr_pid     ON player_attr FIELDS pid;
DEFINE INDEX IF NOT EXISTS idx_attr_pid_key ON player_attr FIELDS pid, key UNIQUE;