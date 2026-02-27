#pragma once

#include "stdafx.h"

// Get the DPI scale factor for a window (1.0 = 96dpi, 1.5 = 144dpi, 2.0 = 192dpi)
inline double GetDpiScale(HWND hWnd = NULL) {
	HDC hdc = ::GetDC(hWnd);
	double scale = ::GetDeviceCaps(hdc, LOGPIXELSX) / 96.0;
	::ReleaseDC(hWnd, hdc);
	return scale;
}

// Scale a pixel value by DPI
inline int DpiScale(int value, HWND hWnd = NULL) {
	return (int)(value * GetDpiScale(hWnd));
}
