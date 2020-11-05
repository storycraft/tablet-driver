.SUFFIXES :

# --------------------
# Config
# --------------------
OUT_DIR := target
SCRIPT_DIR := scripts
SCRIPTS := install.bat uninstall.bat
CONFIGURATOR_DIR := configurator
EXECUTABLE := story-tablet-driver.exe
PACKAGE_NAME := package.zip

# --------------------
# Settings
# --------------------
ifdef DEBUG
BUILD_TYPE := debug
else
BUILD_TYPE := release
endif
PACKAGE_DIR = $(OUT_DIR)/out
BIN_BUILD_DIR = $(OUT_DIR)/$(BUILD_TYPE)

SCRIPT_SRC = $(addprefix $(SCRIPT_DIR)/,$(SCRIPTS))
SCRIPT_DIST = $(addprefix $(PACKAGE_DIR)/,$(SCRIPTS))
EXECUTABLE_SRC = $(addprefix $(BIN_BUILD_DIR)/,$(EXECUTABLE))
EXECUTABLE_DIST = $(addprefix $(PACKAGE_DIR)/,$(EXECUTABLE))
PACKAGE_DIST = $(PACKAGE_DIR)/$(PACKAGE_NAME)
CONFIGURATOR_DIST_DIR = $(PACKAGE_DIR)/$(CONFIGURATOR_DIR)

# --------------------
# Utils
# --------------------
RS = cargo
RS_BUILD_OPT = --target-dir $(OUT_DIR) $(if ifeq $(BUILD_TYPE) release,--release,)
MKDIR = mkdir
CD = cd
CP = cp
RM = rm
ZIP = zip

default : dist

package : dist $(PACKAGE_DIST)

dist : .print_config $(EXECUTABLE_DIST) $(SCRIPT_DIST) $(CONFIGURATOR_DIST_DIR)

$(PACKAGE_DIST) : dist
	@$(CD) $(PACKAGE_DIR) && $(ZIP) $(PACKAGE_NAME) -ur ./
	
$(EXECUTABLE_SRC) :
	$(info Building executable..)
	@$(RS) build $(RS_BUILD_OPT)

$(PACKAGE_DIR)/%.exe : $(BIN_BUILD_DIR)/%.exe | $(PACKAGE_DIR)
	$(info Copying $< to $@)
	@$(CP) -f $< $@

$(PACKAGE_DIR)/% : $(SCRIPT_DIR)/% | $(PACKAGE_DIR)
	$(info Copying $? to $@)
	@$(CP) -f $< $@

$(CONFIGURATOR_DIST_DIR) : $(CONFIGURATOR_DIR) | $(PACKAGE_DIR)
	$(info Copying $? to $@)
	@$(CP) -rf $< $@

$(PACKAGE_DIR) $(CONFIGURATOR_DIST_DIR)/% :
	@$(MKDIR) -p $@

clean :
	$(info Cleaning..)
	@$(RM) -rf $(OUT_DIR)



.print_config :
	$(info ========================================)
	$(info OUT_DIR = $(OUT_DIR))
	$(info BUILD_TYPE = $(BUILD_TYPE))
	$(info SCRIPT_DIR = $(SCRIPT_DIR))
	$(info CONFIGURATOR_DIR = $(CONFIGURATOR_DIR))
	$(info SCRIPTS = $(SCRIPTS))
	$(info EXECUTABLE = $(EXECUTABLE))
	$(info ========================================)