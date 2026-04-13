DEFINE TABLE IF NOT EXISTS player_config SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS pid                   ON player_config TYPE int;

DEFINE INDEX IF NOT EXISTS idx_pid               ON player_config FIELDS pid UNIQUE;
