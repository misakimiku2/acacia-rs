DEFINE TABLE IF NOT EXISTS player_item SCHEMAFULL;

DEFINE FIELD IF NOT EXISTS pid               ON player_item TYPE int;
DEFINE FIELD IF NOT EXISTS sid               ON player_item TYPE int;
DEFINE FIELD IF NOT EXISTS template          ON player_item TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS count             ON player_item TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS flag              ON player_item TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS userdata          ON player_item TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS expiration        ON player_item TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS enhance           ON player_item TYPE option<object> DEFAULT {};
DEFINE FIELD IF NOT EXISTS enhance.level     ON player_item TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS enhance.exp       ON player_item TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS enhance.break     ON player_item TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS enhance.evolue    ON player_item TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS enhance.trust     ON player_item TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS enhance.pro_level ON player_item TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS enhance.spines    ON player_item TYPE array DEFAULT [];
DEFINE FIELD IF NOT EXISTS enhance.spines.*  ON player_item TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS enhance.affixs    ON player_item TYPE array DEFAULT [];
DEFINE FIELD IF NOT EXISTS enhance.affixs.*  ON player_item TYPE int DEFAULT 0;
DEFINE FIELD IF NOT EXISTS slots             ON player_item TYPE object DEFAULT {};
DEFINE FIELD IF NOT EXISTS slots.*           ON player_item TYPE int DEFAULT 0;

DEFINE INDEX IF NOT EXISTS idx_item_pid          ON player_item FIELDS pid;
DEFINE INDEX IF NOT EXISTS idx_item_pid_sid      ON player_item FIELDS pid, sid UNIQUE;
DEFINE INDEX IF NOT EXISTS idx_item_pid_template ON player_item FIELDS pid, template;
DEFINE INDEX IF NOT EXISTS idx_item_pid_slots_4  ON player_item FIELDS pid, slots["4"];