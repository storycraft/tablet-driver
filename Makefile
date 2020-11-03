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
PACKAGE_DIR := $(OUT_DIR)/out
BIN_BUILD_DIR := $(OUT_DIR)/release

SCRIPT_SRC := $(addprefix $(SCRIPT_DIR)/,$(SCRIPTS))
SCRIPT_DIST := $(addprefix $(PACKAGE_DIR)/,$(SCRIPTS))
EXECUTABLE_SRC := $(addprefix $(BIN_BUILD_DIR)/,$(EXECUTABLE))
EXECUTABLE_DIST := $(addprefix $(PACKAGE_DIR)/,$(EXECUTABLE))

# --------------------
# Utils
# --------------------
RS = cargo
RS_BUILD_OPT = --target-dir $(OUT_DIR) --release
MKDIR = mkdir
CP = cp
RM = rm



package : $(PACKAGE_DIR) $(EXECUTABLE_DIST) $(SCRIPT_DIST)

$(EXECUTABLE_DIST) : $(EXECUTABLE_SRC)
	$(CP) $< $(PACKAGE_DIR)

$(EXECUTABLE_SRC) :
	$(RS) build $(RS_BUILD_OPT)
	
$(PACKAGE_DIR) :
	$(MKDIR) $@

$(SCRIPT_DIST) : $(SCRIPT_SRC)
	$(CP) -f $< $@

clean :
	$(RM) -rf $(OUT_DIR)