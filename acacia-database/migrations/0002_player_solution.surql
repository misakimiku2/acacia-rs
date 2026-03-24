DEFINE TABLE IF NOT EXISTS player_solution SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS pid     ON player_solution TYPE int;
DEFINE FIELD IF NOT EXISTS index   ON player_solution TYPE int;
DEFINE FIELD IF NOT EXISTS name    ON player_solution TYPE string DEFAULT "";
DEFINE FIELD IF NOT EXISTS member1 ON player_solution TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS member2 ON player_solution TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS member3 ON player_solution TYPE int DEFAULT 0;

DEFINE INDEX IF NOT EXISTS idx_solution_pid       ON player_solution FIELDS pid;
DEFINE INDEX IF NOT EXISTS idx_solution_pid_index ON player_solution FIELDS pid, index UNIQUE;