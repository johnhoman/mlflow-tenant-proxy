MLFLOW_VERSION ?= 2.7.1
MLFLOW_HOME := $(HOME)/.virtualenvs/mlflow-server
MLFLOW_BIN  := $(MLFLOW_HOME)/bin/mlflow


mlflow.server:
	[ ! -f $(MLFLOW_BIN) ] && $(MLFLOW_HOME)/bin/python -m pip install mlflow==$(MLFLOW_VERSION)
	$(MLFLOW_BIN) server --backend-tracking-uri sqlite://:memory:

proxy:
	cargo run