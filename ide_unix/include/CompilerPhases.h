#ifndef COMPILERPHASES_H
#define COMPILERPHASES_H
#include <wx/notebook.h>
#include <wx/treectrl.h>
#include <wx/wx.h>
#include <wx/vector.h>

class EditorText;
enum{
    LEXER_PHASE = 0,
    SYNTAX_PHASE,
    SEMANTIC_PHASE,
    SYMBOL_TABLE,
    MEDIUM_CODE_PHASE,
    PHASES_COUNT,
};

class CompilerPhases: public wxNotebook
{
    public:
        CompilerPhases(wxWindow *parent, wxWindowID id = wxID_ANY, const wxPoint &pos = wxDefaultPosition, const wxSize &size = wxDefaultSize);
        virtual ~CompilerPhases();
        void SetTreeFromStringArray(wxArrayString& Array);
        void SetSemanticTreeFromStringArray(wxArrayString& Array);

        void SetTextLexic(wxString& Text);
        void SetSymbolTable(wxString& Text);

    protected:

    private:
        EditorText* LexerPhase;
        wxTreeCtrl* SemanticPhase;
        EditorText* SymbolTable;
        EditorText* MediumCodePhase;
        wxTreeCtrl* SyntaxTree;
};

struct tree_item{
    wxTreeItemId Id;
    uint32_t CountSpaces;
};

#endif // COMPILERPHASES_H
