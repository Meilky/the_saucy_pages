CREATE TABLE IF NOT EXISTS recipes_instructions (
	uuid_recipe BLOB NOT NULL,
	uuid_instruction BLOB NOT NULL,
	step_number INTEGER NOT NULL,
	description TEXT NOT NULL,
	PRIMARY KEY (uuid_recipe, uuid_instruction),
	FOREIGN KEY (uuid_recipe) REFERENCES recipes(uuid)
);
