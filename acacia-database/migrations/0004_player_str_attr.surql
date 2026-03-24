DEFINE TABLE IF NOT EXISTS player_str_attr SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS pid   ON player_str_attr TYPE int;
DEFINE FIELD IF NOT EXISTS key   ON player_str_attr TYPE int;
DEFINE FIELD IF NOT EXISTS value ON player_str_attr TYPE string DEFAULT "";

DEFINE INDEX IF NOT EXISTS idx_str_attr_pid     ON player_str_attr FIELDS pid;
DEFINE INDEX IF NOT EXISTS idx_str_attr_pid_key ON player_str_attr FIELDS pid, key UNIQUE;