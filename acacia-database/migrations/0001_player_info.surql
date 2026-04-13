DEFINE TABLE IF NOT EXISTS player_info SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS pid                   ON player_info TYPE int;
DEFINE FIELD IF NOT EXISTS account               ON player_info TYPE string;
DEFINE FIELD IF NOT EXISTS name                  ON player_info TYPE string DEFAULT "";
DEFINE FIELD IF NOT EXISTS sign                  ON player_info TYPE string DEFAULT "";
DEFINE FIELD IF NOT EXISTS level                 ON player_info TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS vigor                 ON player_info TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS charged               ON player_info TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS create_time           ON player_info TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS last_login_time       ON player_info TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS item_id_alloc         ON player_info TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS show_items            ON player_info TYPE array<int> DEFAULT [];
DEFINE FIELD IF NOT EXISTS show_attrs            ON player_info TYPE array<int> DEFAULT [];
DEFINE FIELD IF NOT EXISTS last_pieces           ON player_info TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS last_global_mail_time ON player_info TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS last_person_mid       ON player_info TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS badges                ON player_info TYPE array<int> DEFAULT [];
DEFINE FIELD IF NOT EXISTS serial                ON player_info TYPE int DEFAULT 0;

DEFINE INDEX IF NOT EXISTS idx_pid               ON player_info FIELDS pid UNIQUE;
