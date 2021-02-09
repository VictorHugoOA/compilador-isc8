/***************************************************************
 * Name:      IDE_CompiladorApp.cpp
 * Purpose:   Code for Application Class
 * Author:     ()
 * Created:   2021-02-08
 * Copyright:  ()
 * License:
 **************************************************************/

#include "IDE_CompiladorApp.h"

//(*AppHeaders
#include "IDE_CompiladorMain.h"
#include <wx/image.h>
//*)

IMPLEMENT_APP(IDE_CompiladorApp);

bool IDE_CompiladorApp::OnInit()
{
    //(*AppInitialize
    bool wxsOK = true;
    wxInitAllImageHandlers();
    if ( wxsOK )
    {
    	IDE_CompiladorFrame* Frame = new IDE_CompiladorFrame(0);
    	Frame->Show();
    	SetTopWindow(Frame);
    }
    //*)
    return wxsOK;

}
