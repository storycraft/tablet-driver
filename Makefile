.SUFFIXES :

# --------------------
# Config
# --------------------
OUT_DIR = target
SCRIPT_DIR = scripts
SCRIPTS = install.bat uninstall.bat
EXECUTABLE = story-tablet-driver.exe
PACKAGE_NAME := package.zip

# --------------------
# Settings
# --------------------
ifdef DEBUG
BUILD_TYPE = debug
else
BUILD_TYPE = release
endif
PACKAGE_DIR := $(OUT_DIR)/out
BIN_BUILD_DIR := $(OUT_DIR)/$(BUILD_TYPE)

SCRIPT_SRC := $(addprefix $(SCRIPT_DIR)/,$(SCRIPTS))
SCRIPT_DIST := $(addprefix $(PACKAGE_DIR)/,$(SCRIPTS))
EXECUTABLE_SRC := $(addprefix $(BIN_BUILD_DIR)/,$(EXECUTABLE))
EXECUTABLE_DIST := $(addprefix $(PACKAGE_DIR)/,$(EXECUTABLE))
PACKAGE_DIST := $(PACKAGE_DIR)/$(PACKAGE_NAME)

# --------------------
# Utils
# --------------------
RS = cargo
RS_BUILD_OPT = --target-dir $(OUT_DIR) $(ifeq $(BUILD_TYPE) release,--release,)
MKDIR = mkdir
CD = cd
CP = cp
RM = rm
ZIP = zip

default : dist

package : dist $(PACKAGE_DIST)

dist : .print_config $(EXECUTABLE_DIST) $(SCRIPT_DIST)

$(PACKAGE_DIST) : dist
	@$(CD) $(PACKAGE_DIR) && $(ZIP) $(PACKAGE_NAME) -ur ./
	
$(EXECUTABLE_SRC)/$(EXECUTABLE) :
	$(info Building executable..)
	@$(RS) build $(RS_BUILD_OPT)

$(PACKAGE_DIR)/%.exe : $(BIN_BUILD_DIR)/%.exe | $(PACKAGE_DIR)
	$(info Copying $< to $@)
	@$(CP) -f $< $@

$(PACKAGE_DIR)/% : $(SCRIPT_DIR)/% | $(PACKAGE_DIR)
	$(info Copying $? to $@)
	@$(CP) -f $< $@

$(PACKAGE_DIR) :
	@$(MKDIR) $@

clean :
	$(info Cleaning..)
	@$(RM) -rf $(OUT_DIR)



.print_config :
	@echo ========================================
	@echo OUT_DIR = $(OUT_DIR)
	@echo SCRIPT_DIR = $(SCRIPT_DIR)
	@echo SCRIPTS = $(SCRIPTS)
	@echo EXECUTABLE = $(EXECUTABLE)
	@echo ========================================