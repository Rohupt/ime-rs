// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF
// ANY KIND, EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A
// PARTICULAR PURPOSE.
//
// Copyright (c) Microsoft Corporation. All rights reserved

#include <codecvt>
#include <Windows.h>

#include "Globals.h"
#include "RustStringRange.h"

#pragma comment(lib, "advapi32.lib")
#pragma comment(lib, "ws2_32.lib")
#pragma comment(lib, "userenv.lib")
#pragma comment(lib, "bcrypt.lib")

const DWORD_PTR CStringRangeUtf16::GetLength() const
{
    return _stringBufLen;
}

const WCHAR *CStringRangeUtf16::GetRaw() const
{
    return _pStringBuf.get();
}

void CStringRangeUtf16::SetClone(const WCHAR *pwch, DWORD_PTR dwLength)
{
    _stringBufLen = dwLength;
    _pStringBuf = std::shared_ptr<WCHAR>(Clone(pwch, _stringBufLen));
}

CStringRangeUtf16::CStringRangeUtf16(WCHAR wch)
{
    _stringBufLen = 1;
    _pStringBuf = std::make_shared<WCHAR>(wch);
}

CStringRangeUtf16::CStringRangeUtf16(const CRustStringRange& rsr) {
    // The conversion to UTF16 is in C++ on purpose to allow easier memory management
    static_assert(sizeof(std::wstring::value_type) == sizeof(std::u16string::value_type),
        "std::wstring and std::u16string are expected to have the same character size");
    
    char* firstChar = (char*)rsr.GetRawUtf8();
    // char* afterLastChar = firstChar + rsr.GetLengthUtf8();
    // std::wstring_convert<std::codecvt_utf8_utf16<char16_t>,char16_t> conversion;
    // std::u16string strU16 = conversion.from_bytes(firstChar, afterLastChar);

    int rsrLen = rsr.GetLengthUtf8();
    int u16len = MultiByteToWideChar(CP_UTF8, 0, firstChar, rsrLen, NULL, 0);
    wchar_t* wstr = new wchar_t[u16len];
    MultiByteToWideChar(CP_UTF8, 0, firstChar, rsrLen, wstr, u16len);
    std::u16string strU16(wstr, wstr + u16len);

    SetClone((WCHAR*)strU16.c_str(), strU16.length());
}

CStringRangeUtf16::operator CRustStringRange() const {
    return CRustStringRange(GetRaw(), GetLength());
};

WCHAR* CStringRangeUtf16::Clone(const WCHAR* pwch, DWORD_PTR dwLength)
{
    if (!dwLength) {
        return nullptr;
    }
    WCHAR* pwchString = new (std::nothrow) WCHAR[ dwLength ];
    if (!pwchString)
    {
        return nullptr;
    }
    memcpy((void*)pwchString, pwch, dwLength * sizeof(WCHAR));
    return pwchString;
}
