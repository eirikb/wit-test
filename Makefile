build-module:
	@echo "Building module..."
	cd my-wit-module && cargo component build

run-module:
	@echo "Running module..."
	cd my-wit-run && cargo run

run-module-node:
	@echo "Running module..."
	cd my-wit-node-run && npm i && npm run generate && npm start
