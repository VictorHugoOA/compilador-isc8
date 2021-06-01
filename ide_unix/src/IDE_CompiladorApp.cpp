/***************************************************************
 * Name:      IDE_CompiladorApp.cpp
 * Purpose:   Code for Application Class
 * Author:     ()
 * Created:   2021-02-08
 * Copyright:  ()
 * License:
 **************************************************************/

#include "./../include/IDE_CompiladorApp.h"

#include "./../include/IDE_CompiladorMain.h"
#include <wx/image.h>

IMPLEMENT_APP(IDE_CompiladorApp);

bool IDE_CompiladorApp::OnInit()
{

//Just initialize from fro text editor.
    appFrame = new IDE_CompiladorFrame(nullptr);
    appFrame->Layout();
    appFrame->Show(true);

    return true;

}

int IDE_CompiladorApp::OnExit(){

    return 0;

}
