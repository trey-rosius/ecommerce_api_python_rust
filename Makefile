BASE := $(shell /bin/pwd)
CODE_COVERAGE = 72
PIPENV ?= pipenv
ARCH := aarch64-unknown-linux-gnu
STACK_NAME ?= ecommerce-api-rust

#############
#  SAM vars	#
#############

# Name of Docker Network to connect to
# Helpful when you're running Amazon DynamoDB local etc.
NETWORK = ""

setup:
ifeq (,$(shell which rustc))
	$(error "Could not find Rust compiler, please install it")
endif
ifeq (,$(shell which cargo))
	$(error "Could not find Cargo, please install it")
endif
ifeq (,$(shell which zig))
	$(error "Could not find Zig compiler, please install it")
endif
	cargo install cargo-lambda
ifeq (,$(shell which sam))
	$(error "Could not find SAM CLI, please install it")
endif
target:
	$(info ${HELP_MESSAGE})
	@exit 0

clean: ##=> Deletes current build environment and latest build
	$(info [*] Who needs all that anyway? Destroying environment....)
	rm -rf ./.aws-sam/

all: clean build

install:
	$(info [*] Installing pipenv)
	@pip install pipenv --upgrade
	$(MAKE) dev

dev:
	$(info [*] Installing pipenv project dependencies)
	@$(PIPENV) install
	@$(PIPENV) install -d

shell:
	@$(PIPENV) shell

build: ##=> Same as package except that we don't create a ZIP
##=> sam build --use-container
	cargo lambda build --release --target $(ARCH)
deploy:
	if [ -f samconfig.toml ]; \
		then sam deploy --stack-name $(STACK_NAME); \
		else sam deploy -g --stack-name $(STACK_NAME); \
	fi
run: ##=> Run SAM Local API GW and can optionally run new containers connected to a defined network
	@test -z ${NETWORK} \
		&& sam local start-api \
		|| sam local start-api --docker-network ${NETWORK}

test: ##=> Run pytest
	POWERTOOLS_METRICS_NAMESPACE="MyServerlessApplication" $(PIPENV) run python -m pytest --cov . --cov-report term-missing --cov-fail-under $(CODE_COVERAGE) tests/ -vv

ci: ##=> Run full workflow - Install deps, build deps, and deploy
	$(MAKE) dev
	$(MAKE) build
	$(MAKE) deploy

hurry: ##=> Run full workflow for the first time
	$(MAKE) install
	$(MAKE) build
	$(MAKE) deploy.guided

#############
#  Helpers  #
#############

define HELP_MESSAGE
	Environment variables to be aware of or to hardcode depending on your use case:

	NETWORK
		Default: ""
		Info: Docker Network to connect to when running Lambda function locally

	Common usage:

	...::: Installs Pipenv, application and dev dependencies defined in Pipfile :::...
	$ make install

	...::: Builds Lambda function dependencies:::...
	$ make build

	...::: Deploy for the first time :::...
	$ make deploy.guided

	...::: Deploy subsequent changes :::...
	$ make deploy

	...::: Run SAM Local API Gateway :::...
	$ make run

	...::: Run Pytest under tests/ with pipenv :::...
	$ make test

	...::: Spawn a virtual environment shell :::...
	$ make shell

	...::: Cleans up the environment - Deletes Virtualenv, ZIP builds and Dev env :::...
	$ make clean

	...::: Run full workflow from installing Pipenv, dev and app deps, build, and deploy :::...
	$ make ci

	Advanced usage:

	...::: Run SAM Local API Gateway within a Docker Network :::...
	$ make run NETWORK="sam-network"
endef
