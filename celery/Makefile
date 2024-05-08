include .env

ifeq ($(OS),Windows_NT) 
    detected_OS := Windows
	VENV_BIN_DIR = venv/Scripts
else
    detected_OS := $(shell sh -c 'uname 2>/dev/null || echo Unknown')
	VENV_BIN_DIR = venv/bin
endif

CMD_FROM_VENV = ". $(VENV_BIN_DIR)/activate; which"
PYTHON = "$(VENV_BIN_DIR)/python"
PIP = "$(VENV_BIN_DIR)/pip"
define create-venv
python3 -m venv venv
endef
define create-venv-win
$(PYTHON_ROOT)/python.exe -m venv venv
endef

.PHONY: all
all: run-py

task:
	$(PYTHON) app.py

celery:
	$(PYTHON) -m celery -A tasks worker --loglevel=INFO

sqlite:
	sqlite3 db.sqlite3
	# .tables
	# select * from celery_taskmeta;

venv:
	@$(create-venv)
	@$(PIP) install -r requirements.txt

venv-win:
	@$(create-venv-win)
	@$(PIP) install -r requirements.txt

clean-venv:
	@rm -rf venv