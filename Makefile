.SUFFIXES :

# --------------------
# Config
# --------------------
OUT_DIR = target
SCRIPT_DIR = scripts
SCRIPTS = install.bat uninstall.bat
EXECUTABLE = story-tablet-driver.exe

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

# --------------------
# Utils
# --------------------
RS = cargo
RS_BUILD_OPT = --target-dir $(OUT_DIR) $(ifeq $(BUILD_TYPE) release,--release,)
MKDIR = mkdir
CP = cp
RM = rm

package : .print_config $(PACKAGE_DIR) $(EXECUTABLE_DIST) $(SCRIPT_DIST)

$(EXECUTABLE_DIST) : $(EXECUTABLE_SRC)
	@echo 'Copying $< to $@'
	@$(CP) $< $(PACKAGE_DIR)

$(EXECUTABLE_SRC) :
	@echo Building executable..
	@$(RS) build $(RS_BUILD_OPT)
	
$(PACKAGE_DIR) :
	@$(MKDIR) $@

$(SCRIPT_DIST) : $(SCRIPT_SRC)
	@echo 'Copying $< to $@'
	@$(CP) -f $< $@

clean :
	@echo Cleaning..
	@$(RM) -rf $(OUT_DIR)



.print_config :
	@echo ========================================
	@echo OUT_DIR = $(OUT_DIR)
	@echo SCRIPT_DIR = $(SCRIPT_DIR)
	@echo SCRIPTS = $(SCRIPTS)
	@echo EXECUTABLE = $(EXECUTABLE)
	@echo ========================================