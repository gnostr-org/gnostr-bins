ifeq ($(TAG),)
TAG := v$(shell cat Cargo.toml | grep 'version = "' | head -n 1 | sed 's/version = "\(.*\)".*/\1/')
endif
export TAG



help:
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?##/ {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
	@echo

##
##===============================================================================
##all
## 	bin
all: 	bin### 	all
##bin
## 	cargo b --manifest-path Cargo.toml
bin: 	### 	bin
	cargo b --manifest-path Cargo.toml

##
##===============================================================================
##make cargo-*
cargo-help: 	### 	cargo-help
	@awk 'BEGIN {FS = ":.*?###"} /^[a-zA-Z_-]+:.*?###/ {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
cargo-release-all: 	### 	cargo-release-all
## 	cargo-release-all recursively cargo build --release
	for t in **Cargo.toml;  do echo $$t; cargo b -r -vv --manifest-path $$t; done
cargo-clean-release: 	### 	cargo-clean-release - clean release artifacts
## 	cargo-clean-release 	recursively cargo clean --release
	for t in **Cargo.toml;  do echo $$t && cargo clean --release -vv --manifest-path $$t 2>/dev/null; done
cargo-publish-all: 	### 	cargo-publish-all
## 	cargo-publish-all 	recursively publish rust projects
	for t in *Cargo.toml;  do echo $$t; cargo publish -vv --manifest-path $$t; done

cargo-i:cargo-install### 	cargo-i
cargo-install-bins:### 	cargo-install-bins
## 	cargo-install-all 	recursively cargo install -vv $(SUBMODULES)
## 	*** cargo install -vv --force is NOT used.
## 	*** FORCE=--force cargo install -vv $(FORCE) is used.
## 	*** FORCE=--force cargo install -vv $(FORCE) --path <path>
## 	*** to overwrite deploy cargo.io crates.
	export RUSTFLAGS=-Awarning;  for t in $(SUBMODULES); do echo $$t; cargo install --bins --path  $$t -vv $(FORCE) 2>/dev/null || echo ""; done
	#for t in $(SUBMODULES); do echo $$t; cargo install -vv gnostr-$$t --force || echo ""; done

cargo-build: 	## 	cargo build
## 	cargo-build q=true
	@. $(HOME)/.cargo/env
	@RUST_BACKTRACE=all cargo b $(QUIET)
cargo-install: 	### 	cargo install --path . $(FORCE)
	@. $(HOME)/.cargo/env
	@cargo install --path . $(FORCE)
## 	cargo-br q=true
cargo-build-release: 	### 	cargo-build-release
## 	cargo-build-release q=true
	@. $(HOME)/.cargo/env
	@cargo b --release $(QUIET)
cargo-check: 	### 	cargo-check
	@. $(HOME)/.cargo/env
	@cargo c
cargo-bench: 	### 	cargo-bench
	@. $(HOME)/.cargo/env
	@cargo bench
cargo-test: 	### 	cargo-test
	@. $(HOME)/.cargo/env
	#@cargo test
	cargo test
cargo-test-nightly: 	### 	cargo-test-nightly
	@. $(HOME)/.cargo/env
	#@cargo test
	cargo +nightly test
cargo-report: 	### 	cargo-report
	@. $(HOME)/.cargo/env
	cargo report future-incompatibilities --id 1
cargo-run: 	### 	cargo-run
	@. $(HOME)/.cargo/env
	cargo run --bin gnostr -- -h

##===============================================================================
cargo-dist: 	### 	make cargo-dist TAG=$(TAG)
	
	@dist host --steps=create --tag=$(TAG) --allow-dirty --output-format=json > plan-dist-manifest.json
cargo-dist-build: 	### 	cargo-dist-build
	RUSTFLAGS="--cfg tokio_unstable" cargo dist build
cargo-dist-manifest: 	### 	cargo dist manifest --artifacts=all
	cargo dist manifest --artifacts=all

t: cargo-i
	gnostr-get-relays --nip 111 -s ## args.len() == 4
	gnostr-get-relays --nip 111    ## args.len() == 3
	gnostr-get-relays --nip        ## args.len() == 2
	gnostr-get-relays              ## args.len() == 1
all-tests: test-loop-back test-gnostr-get-relays##     all-tests
test-loop-back:##  test-loop-back
	gnostr-fetch-metadata wss://relay.damus.io a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd | gnostr-post-event
test-gnostr-get-relays:##      test-gnostr-get-relays
	cargo install --bin gnostr-get-relays --path .
	for relay in $$(gnostr-get-relays -s); do echo $$relay;done
	for relay in $$(gnostr-get-relays --nip 111 -s); do echo $$relay;done
test-gnostr-fetch-watch-list-iterator:##       test-gnostr-fetch-watch-list-iterator
	cargo install --bin gnostr-fetch-watch-list-iterator --path .
	for relay in $$(gnostr-fetch-watch-list-iterator); do echo $$relay;done
test-gnostr-post-duplicate:##  test-gnostr-post-duplicate
	@[ -x $(shell which cat) ] && \
               cat tests/event.ab0d7c747e0d6651814f8092287f9a58c9cc7a48ce700e2cf743c082577f7850 | gnostr-post-event --relay wss://relay.damus.io
test-gnostr-post-commit:##     test-gnostr-post-commit
	@[ -x $(shell which cat) ] && \
               cat tests/first-gnostr-commit.json | gnostr-post-event --relay wss://relay.damus.io
test-gnostr-fetch-first-commit:##      test-gnostr-fetch-first-commit
	@gnostr-fetch-by-id wss://relay.damus.io fbf73a17a4e0fe390aba1808a8d55f1b50717d5dd765b2904bf39eba18c51f7c
test-gnostr-post-event:##      test-gnostr-post-event
	@cargo install --bin gnostr-post-event --path . && \
       [ -x $(shell which gnostr) ] && [ -x $(shell which gnostr-sha256) ] && \
       [ -x $(shell which gnostr-weeble) ] && [ -x $(shell which gnostr-wobble) ] && \
       [ -x $(shell which gnostr-blockheight) ] && \
               gnostr --sec $(shell gnostr-sha256 $(shell gnostr-weeble)) -t gnostr --tag weeble $(shell gnostr-weeble) --tag wobble $(shell gnostr-wobble) --tag blockheight $(shell gnostr-blockheight) --content 'gnostr/$(shell gnostr-weeble)/$(shell gnostr-blockheight)/$(shell gnostr-wobble))' | cargo run --bin gnostr-post-event -- --relay wss://nos.lol
test-gnostr-post-event-context:##      test-gnostr-post-event-context
	@cargo install --bin gnostr-post-event --path . && \
       [ -x $(shell which gnostr) ] && [ -x $(shell which gnostr-sha256) ] && \
       [ -x $(shell which gnostr-weeble) ] && [ -x $(shell which gnostr-wobble) ] && \
       [ -x $(shell which gnostr-blockheight) ] && \
               gnostr --sec $(shell gnostr-sha256 $(shell gnostr-weeble)) -t gnostr --tag weeble $(shell gnostr-weeble) --tag wobble $(shell gnostr-wobble) --tag blockheight $(shell gnostr-blockheight) --content 'gnostr/$(shell gnostr-weeble)/$(shell gnostr-blockheight)/$(shell gnostr-wobble))' | cargo run --bin gnostr-post-event -- --relay wss://nos.lol
test-gnostr-bounce-event:##    test-gnostr-bounce-event
	make test-gnostr-fetch-first-commit | gnostr-post-event
# vim: set noexpandtab:
# vim: set setfiletype make
