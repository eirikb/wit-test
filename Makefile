build-module:
	@echo "Building module..."
	@cd my-wit-module && cargo component build --target wasm32-unknown-unknown

run-module: build-module
	@echo "Running module..."
	@cd my-wit-run && cargo run

run-module-node: build-module
	@echo "Running module..."
	@cd my-wit-node-run && npm install && npm run generate && npm start
