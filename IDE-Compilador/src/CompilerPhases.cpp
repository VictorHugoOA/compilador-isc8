#include "CompilerPhases.h"
#include "EditorText.h"

CompilerPhases::CompilerPhases(wxWindow *parent, wxWindowID id, const wxPoint &pos, const wxSize &size): wxNotebook(parent, id, pos, size)
{
    for(int i = 0; i < PHASES_COUNT; ++i)
    {
        phases[i] = new EditorText(this);
    }

    InsertPage(LEXER_PHASE, phases[LEXER_PHASE], "Lexic", true);
    InsertPage(SYNTAX_PHASE, phases[SYNTAX_PHASE], "Syntax", false);
    InsertPage(SEMANTIC_PHASE, phases[SEMANTIC_PHASE], "Semantic", false);
    InsertPage(MEDIUM_CODE_PHASE, phases[MEDIUM_CODE_PHASE], "Medium code", false);

}

CompilerPhases::~CompilerPhases()
{
    //dtor
}
