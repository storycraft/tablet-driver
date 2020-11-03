RS = cargo
RS_BUILD_OPT = --target-dir $(OUT_DIR) --release



OUT_DIR = target
SCRIPT_DIR = scripts
SCRIPTS = install.bat uninstall.bat
EXECUTABLE = story-tablet-driver.exe

PACKAGE_DIR := $(OUT_DIR)/out
BIN_BUILD_DIR := $(OUT_DIR)/release

SCRIPT_SRC := $(addprefix $(SCRIPT_DIR)/,$(SCRIPTS))
SCRIPT_DIST := $(addprefix $(PACKAGE_DIR)/,$(SCRIPTS))
EXECUTABLE_SRC := $(addprefix $(BIN_BUILD_DIR)/,$(EXECUTABLE))
EXECUTABLE_DIST := $(addprefix $(PACKAGE_DIR)/,$(EXECUTABLE))



package : $(PACKAGE_DIR) $(EXECUTABLE_DIST) $(SCRIPT_DIST)

$(EXECUTABLE_DIST) : $(EXECUTABLE_SRC)
	cp $< $(PACKAGE_DIR)

$(EXECUTABLE_SRC) :
	$(RS) build $(RS_BUILD_OPT)
	
$(PACKAGE_DIR) :
	mkdir $@

$(SCRIPT_DIST) : $(SCRIPT_SRC)
	cp -f $< $@

clean :
	rm -rf $(OUT_DIR)