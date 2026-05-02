DEFINE TABLE IF NOT EXISTS player_config SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS pid                   ON player_config TYPE int;
DEFINE FIELD IF NOT EXISTS token                 ON player_config TYPE string;
DEFINE FIELD IF NOT EXISTS ip                    ON player_config TYPE string DEFAULT "";
DEFINE FIELD IF NOT EXISTS sdk_username          ON player_config TYPE string DEFAULT "";
DEFINE FIELD IF NOT EXISTS sdk_password          ON player_config TYPE string DEFAULT "";
DEFINE FIELD IF NOT EXISTS sdk_phone_number      ON player_config TYPE string DEFAULT "";

DEFINE INDEX IF NOT EXISTS idx_pid               ON player_config FIELDS pid UNIQUE;
