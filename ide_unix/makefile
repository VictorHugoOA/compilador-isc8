CXX = `$(WX_CONFIG) --cxx`
CXXFLAGS ?= -std=c++11
LD_FLAGS ?=

WX_CONFIG ?= wx-config
WX_UNICODE ?= 1
WX_SHARED = 1
WX_VERSION ?= $(shell $(WX_CONFIG) --query-version | sed -e 's/\([0-9]*\)\.\([0-9]*\)/\1\2/') 
WX_PORT ?= $(shell $(WX_CONFIG) --query-toolkit)

WX_CONFIG_UNICODE_FLAG = --unicode=yes
WX_CONFIG_SHARED_FLAG = --static=no

CPPDEPS = -MT$@ -MF`echo $@ | sed -e 's,\.o$$,.d,'` -MD -MP
WX_VERSION_MAJOR = $(shell echo $(WX_VERSION) | cut -c1,1)
WX_VERSION_MINOR = $(shell echo $(WX_VERSION) | cut -c2,2)
WX_CONFIG_FLAGS = $(WX_CONFIG_UNICODE_FLAG) $(WX_CONFIG_SHARED_FLAG) --toolkit=$(WX_PORT) --version=$(WX_VERSION_MAJOR).$(WX_VERSION_MINOR)
IDE_CXXFLAGS = -I. `$(WX_CONFIG) --cxxflags $(WX_CONFIG_FLAGS)` $(CXXFLAGS)

IDE_OBJECTS = CompilerPhases.o EditorText.o IDE_CompiladorApp.o IDE_CompiladorMain.o prefs.o ResultsShow.o

all: ide

ide: $(IDE_OBJECTS)
	$(CXX) -o $@ $(IDE_OBJECTS) `$(WX_CONFIG) $(WX_CONFIG_FLAGS) --libs stc,core,base`

ResultsShow.o: ./src/ResultsShow.cpp
	$(CXX) -c -o $@ $(IDE_CXXFLAGS) $(CPPDEPS) $<

prefs.o: ./src/prefs.cpp
	$(CXX) -c -o $@ $(IDE_CXXFLAGS) $(CPPDEPS) $<

IDE_CompiladorMain.o: ./src/IDE_CompiladorMain.cpp
	$(CXX) -c -o $@ $(IDE_CXXFLAGS) $(CPPDEPS) $<

IDE_CompiladorApp.o: ./src/IDE_CompiladorApp.cpp
	$(CXX) -c -o $@ $(IDE_CXXFLAGS) $(CPPDEPS) $<

EditorText.o: ./src/EditorText.cpp
	$(CXX) -c -o $@ $(IDE_CXXFLAGS) $(CPPDEPS) $<

CompilerPhases.o: ./src/CompilerPhases.cpp
	$(CXX) -c -o $@ $(IDE_CXXFLAGS) $(CPPDEPS) $<