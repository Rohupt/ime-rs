// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//
// Copyright (c) Microsoft Corporation. All rights reserved

#include "Private.h"
#include "KeyHandlerEditSession.h"
#include "EditSession.h"
#include "SampleIME.h"
#include "CompositionProcessorEngine.h"
#include "KeyStateCategory.h"
#include <sstream>

//////////////////////////////////////////////////////////////////////
//
//    ITfEditSession
//        CEditSessionBase
// CKeyHandlerEditSession class
//
//////////////////////////////////////////////////////////////////////

//+---------------------------------------------------------------------------
//
// CKeyHandlerEditSession::DoEditSession
//
//----------------------------------------------------------------------------

STDAPI CKeyHandlerEditSession::DoEditSession(TfEditCookie ec)
{
    HRESULT hResult = S_OK;

    CKeyStateCategory* pKeyStateCategory = CKeyStateCategoryFactory::MakeKeyStateCategory(_KeyState.Category, _pTextService);

    if (pKeyStateCategory)
    {
        KeyHandlerEditSessionDTO keyHandlerEditSessionDTO(ec, _pContext, _wch, _KeyState.Function);
        hResult = pKeyStateCategory->KeyStateHandler(_KeyState.Function, keyHandlerEditSessionDTO);

        pKeyStateCategory->Release();
    }

    return hResult;
}
