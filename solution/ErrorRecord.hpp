#include <windows.h>

struct ErrorRecord
{
    PCWSTR Message;
    DWORD ErrorCode;
    DWORD LineNumber;
    PCWSTR File;
};

void
ReportError(ErrorRecord Rec);