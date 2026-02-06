CREATE TABLE IF NOT EXISTS recipes_ingredients (
	uuid_recipe BLOB NOT NULL,
	uuid_ingredient BLOB NOT NULL,
	amount FLOAT NOT NULL,
	PRIMARY KEY (uuid_recipe, uuid_ingredient),
	FOREIGN KEY (uuid_recipe) REFERENCES recipes(uuid),
	FOREIGN KEY (uuid_ingredient) REFERENCES ingredients(uuid)
);
