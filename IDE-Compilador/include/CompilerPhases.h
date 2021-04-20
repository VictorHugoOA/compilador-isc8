#ifndef COMPILERPHASES_H
#define COMPILERPHASES_H
#include <wx/notebook.h>

class EditorText;
enum{
    LEXER_PHASE = 0,
    SYNTAX_PHASE,
    SEMANTIC_PHASE,
    MEDIUM_CODE_PHASE,
    PHASES_COUNT
};

class CompilerPhases: public wxNotebook
{
    public:
        CompilerPhases(wxWindow *parent, wxWindowID id = wxID_ANY, const wxPoint &pos = wxDefaultPosition, const wxSize &size = wxDefaultSize);
        virtual ~CompilerPhases();

        void SetText(wxString& Text, int Phase);

    protected:

    private:
        EditorText* phases[PHASES_COUNT];
};

#endif // COMPILERPHASES_H
