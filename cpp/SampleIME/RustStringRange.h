#pragma once

#include "..\..\rust\ruststringrange\ruststringrange.h"

#include "SampleIMEBaseStructure.h"

/**
 * A C++ wrapper class for its Rust counterpart
 */
class CRustStringRange {
  void Set(const wchar_t* pwch, uintptr_t dwLength) {
    range = ruststringrange_new((const uint16_t*)pwch, dwLength);
  }
  explicit CRustStringRange(void* range_raw) {
    range = range_raw;
  }

 public:
  static CRustStringRange from_void(void* range_raw) {
    return CRustStringRange(range_raw);
  }
  CRustStringRange(const CRustStringRange& that) {
    range = ruststringrange_clone(that.range);
  }
  CRustStringRange(CRustStringRange&& that) noexcept {
    range = that.range;
    that.range = nullptr;
  }
  explicit CRustStringRange(const CStringRangeBase& cstr) {
    Set(cstr.GetRaw(), cstr.GetLength());
  }
  CRustStringRange(const wchar_t* pwch, uintptr_t dwLength) {
    Set(pwch, dwLength);
  }
  CRustStringRange(const char* pch, uintptr_t dwLength) {
    range = ruststringrange_new_utf8((const uint8_t*)pch, dwLength);
  }

  ~CRustStringRange() {
    if (range) {
      ruststringrange_free(range);
    }
  }

  CRustStringRange& operator=(CRustStringRange sr) {
    std::swap(range, sr.range);
    return *this;
  }

  bool operator==(const CRustStringRange& sr) const {
    return Compare(sr) == 0;
  }

  bool operator!=(const CRustStringRange& sr) const {
    return !(*this == sr);
  }

  bool operator>(const CRustStringRange& sr) const {
    return Compare(sr) > 0;
  }

  bool operator>=(const CRustStringRange& sr) const {
    return Compare(sr) >= 0;
  }

  bool operator<(const CRustStringRange& sr) const {
    return Compare(sr) < 0;
  }

  bool operator<=(const CRustStringRange& sr) const {
    return Compare(sr) <= 0;
  }

  int8_t Compare(const CRustStringRange& sr) const {
    return ruststringrange_compare(this->range, sr.range);
  }

  uintptr_t GetLengthUtf8() const {
    return ruststringrange_len(range);
  }

  const uint8_t* GetRawUtf8() const {
    return ruststringrange_raw(range);
  }

  void* GetInternal() const {
    return range;
  }

  bool CompareWithWildCard(const CRustStringRange& target) const {
    return ruststringrange_compare_with_wildcard(this->range, target.range);
  }

  CRustStringRange Concat(const CRustStringRange& sr) const {
    return CRustStringRange(ruststringrange_concat(this->range, sr.range));
  }

  bool Contains(char ch) {
    return ruststringrange_contains(this->range, ch);
  }

  CRustStringRange CutLast() {
    return CRustStringRange(ruststringrange_cutlast(range));
  }

private:
  void* range = nullptr;
};

CRustStringRange operator""_rs(const char* aStr, std::size_t aLen);
