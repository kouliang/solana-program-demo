.PHONY: all

all: deploy hello greet account pda transfer

clean:
	@if [ -d "target/deploy" ]; then \
		echo "Removing directory..."; \
		rm -rf target/deploy; \
	fi

deploy: clean
	cargo build-sbf
	solana program deploy target/deploy/kl.so

upgread:
	cargo build-sbf
	solana program deploy target/deploy/kl.so

hello:
	cargo run --example hello

greet:
	cargo run --example greet

account:
	cargo run --example account

pda:
	cargo run --example pda

transfer:
	cargo run --example transfer
