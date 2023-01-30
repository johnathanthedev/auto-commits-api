db-migrate:
	sea-orm-cli migrate refresh;

db-rollback:
	sea-orm-cli migrate down;

generate-entities:
	sea-orm-cli generate entity -o src/entities;