#include "./../include/CompilerPhases.h"
#include "./../include/EditorText.h"

uint32_t GetStartSpacesCount(wxString& String){
    uint32_t Result = 0;
    while(String[Result++] == ' ');

    return (Result - 1);
}

CompilerPhases::CompilerPhases(wxWindow *parent, wxWindowID id, const wxPoint &pos, const wxSize &size): wxNotebook(parent, id, pos, size)
{
    LexerPhase = new EditorText(this);
    SemanticPhase = new EditorText(this);
    MediumCodePhase = new EditorText(this);
    SyntaxTree = new wxTreeCtrl(this, wxID_ANY);

    InsertPage(LEXER_PHASE, LexerPhase, "Lexic", true);
    InsertPage(SYNTAX_PHASE, SyntaxTree, "Syntax", false);
    InsertPage(SEMANTIC_PHASE, SemanticPhase, "Semantic", false);
    InsertPage(MEDIUM_CODE_PHASE, MediumCodePhase, "Medium code", false);

}

void CompilerPhases::SetTreeFromStringArray(wxArrayString& Array){
    SyntaxTree->DeleteAllItems();

    wxVector<tree_item> Stack;

    size_t Index = 0;
    uint32_t SpaceCount = GetStartSpacesCount(Array[Index]);
    wxTreeItemId Root = SyntaxTree->AddRoot(Array[Index++].Trim(false));

    tree_item RootItem = {Root, SpaceCount};
    Stack.push_back(RootItem);
    tree_item CurrentItem = Stack.back();
    size_t CountStrings = Array.GetCount();

    for(; Index < CountStrings; ++Index){
        uint32_t SpacesItem = GetStartSpacesCount(Array[Index]);
        if(SpacesItem > CurrentItem.CountSpaces){
            wxTreeItemId ItemId = SyntaxTree->AppendItem(CurrentItem.Id, Array[Index].Trim(false));
            tree_item NewItem = {ItemId, SpacesItem};
            Stack.push_back(NewItem);
            CurrentItem = Stack.back();
        }
        else{
            while(CurrentItem.CountSpaces >= SpacesItem){
                Stack.pop_back();
                CurrentItem = Stack.back();
            }

            wxTreeItemId ItemId = SyntaxTree->AppendItem(CurrentItem.Id, Array[Index].Trim(false));
            tree_item NewItem = {ItemId, SpacesItem};
            Stack.push_back(NewItem);
            CurrentItem = Stack.back();
        }
    }

}

void CompilerPhases::SetTextLexic(wxString &Text){
    LexerPhase->SetReadOnly(false);
    LexerPhase->SetText(Text);
    LexerPhase->SetReadOnly(true);
}

CompilerPhases::~CompilerPhases()
{
    //dtor
}
