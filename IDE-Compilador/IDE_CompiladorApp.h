/***************************************************************
 * Name:      IDE_CompiladorApp.h
 * Purpose:   Defines Application Class
 * Author:     ()
 * Created:   2021-02-08
 * Copyright:  ()
 * License:
 **************************************************************/

#ifndef IDE_COMPILADORAPP_H
#define IDE_COMPILADORAPP_H
#include "IDE_CompiladorMain.h"

#include <wx/app.h>

class IDE_CompiladorFrame;

class IDE_CompiladorApp : public wxApp
{
    public:
        virtual bool OnInit();
        virtual int OnExit();
    private:
        IDE_CompiladorFrame* appFrame;

};

#endif // IDE_COMPILADORAPP_H
