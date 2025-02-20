#ifndef RUST_EXPORTS_H
#define RUST_EXPORTS_H

#include "main.h"

enum SummaryScreenStyle {
  SUMMARY_SCREEN_STYLE_PARTY = 0,
  SUMMARY_SCREEN_STYLE_READ_ONLY = 1,
};

void InitFullSummaryScreen(MainCallback cb, enum SummaryScreenStyle style,
                           struct Pokemon *mons, u32 count);

#endif