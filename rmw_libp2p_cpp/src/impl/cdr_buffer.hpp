// Copyright 2022 Esteve Fernandez All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef IMPL__CDR_BUFFER_HPP_
#define IMPL__CDR_BUFFER_HPP_

#include <algorithm>
#include <limits>
#include <stdexcept>
#include <string>
#include <vector>

#include "rcutils/logging_macros.h"

#include "impl/rmw_libp2p_rs.hpp"

namespace rmw_libp2p_cpp
{
namespace cdr
{
class WriteCDRBuffer
{
public:
  WriteCDRBuffer()
  {
    buffer_ = rs_libp2p_cdr_buffer_write_new();
  }
  ~WriteCDRBuffer()
  {
    rs_libp2p_cdr_buffer_free(buffer_);
  }

  const rs_libp2p_cdr_buffer * data() const noexcept {return buffer_;}

  inline WriteCDRBuffer & operator<<(const uint64_t n)
  {
    rs_libp2p_cdr_buffer_write_uint64(buffer_, n);
    return *this;
  }
  inline WriteCDRBuffer & operator<<(const uint32_t n)
  {
    rs_libp2p_cdr_buffer_write_uint32(buffer_, n);
    return *this;
  }
  inline WriteCDRBuffer & operator<<(const uint16_t n)
  {
    rs_libp2p_cdr_buffer_write_uint16(buffer_, n);
    return *this;
  }
  inline WriteCDRBuffer & operator<<(const uint8_t n)
  {
    rs_libp2p_cdr_buffer_write_uint8(buffer_, n);
    return *this;
  }

  inline WriteCDRBuffer & operator<<(const int64_t n)
  {
    rs_libp2p_cdr_buffer_write_int64(buffer_, n);
    return *this;
  }
  inline WriteCDRBuffer & operator<<(const int32_t n)
  {
    rs_libp2p_cdr_buffer_write_int32(buffer_, n);
    return *this;
  }
  inline WriteCDRBuffer & operator<<(const int16_t n)
  {
    rs_libp2p_cdr_buffer_write_int16(buffer_, n);
    return *this;
  }
  inline WriteCDRBuffer & operator<<(const int8_t n)
  {
    rs_libp2p_cdr_buffer_write_int8(buffer_, n);
    return *this;
  }

  inline WriteCDRBuffer & operator<<(const char n)
  {
    rs_libp2p_cdr_buffer_write_char(buffer_, n);
    return *this;
  }
  inline WriteCDRBuffer & operator>>(char16_t n)
  {
    rs_libp2p_cdr_buffer_write_char16(buffer_, n);
    return *this;
  }
  inline WriteCDRBuffer & operator<<(const float f)
  {
    rs_libp2p_cdr_buffer_write_float(buffer_, f);
    return *this;
  }
  inline WriteCDRBuffer & operator<<(const double d)
  {
    rs_libp2p_cdr_buffer_write_double(buffer_, d);
    return *this;
  }
  inline WriteCDRBuffer & operator<<(const bool b)
  {
    rs_libp2p_cdr_buffer_write_bool(buffer_, b);
    return *this;
  }
  inline WriteCDRBuffer & operator<<(const std::string & s)
  {
    rs_libp2p_cdr_buffer_write_string(buffer_, s.c_str(), s.length());
    return *this;
  }
  inline WriteCDRBuffer & operator<<(const std::u16string & s)
  {
    rs_libp2p_cdr_buffer_write_u16string(
      buffer_, reinterpret_cast<const char16_t *>(s.data()), s.length());
    return *this;
  }

private:
  rs_libp2p_cdr_buffer * buffer_;
};

class ReadCDRBuffer
{
public:
  explicit ReadCDRBuffer(const uint8_t * data, uintptr_t length)
  {
    buffer_ = rs_libp2p_cdr_buffer_read_new(data, length);
  }
  ~ReadCDRBuffer()
  {
    rs_libp2p_cdr_buffer_free(buffer_);
  }
  inline ReadCDRBuffer & operator>>(uint64_t & n)
  {
    rs_libp2p_cdr_buffer_read_uint64(buffer_, &n);
    return *this;
  }
  inline ReadCDRBuffer & operator>>(uint32_t & n)
  {
    rs_libp2p_cdr_buffer_read_uint32(buffer_, &n);
    return *this;
  }
  inline ReadCDRBuffer & operator>>(uint16_t & n)
  {
    rs_libp2p_cdr_buffer_read_uint16(buffer_, &n);
    return *this;
  }
  inline ReadCDRBuffer & operator>>(uint8_t & n)
  {
    rs_libp2p_cdr_buffer_read_uint8(buffer_, &n);
    return *this;
  }
  inline ReadCDRBuffer & operator>>(int64_t & n)
  {
    rs_libp2p_cdr_buffer_read_int64(buffer_, &n);
    return *this;
  }
  inline ReadCDRBuffer & operator>>(int32_t & n)
  {
    rs_libp2p_cdr_buffer_read_int32(buffer_, &n);
    return *this;
  }
  inline ReadCDRBuffer & operator>>(int16_t & n)
  {
    rs_libp2p_cdr_buffer_read_int16(buffer_, &n);
    return *this;
  }
  inline ReadCDRBuffer & operator>>(int8_t & n)
  {
    rs_libp2p_cdr_buffer_read_int8(buffer_, &n);
    return *this;
  }
  inline ReadCDRBuffer & operator>>(char & n)
  {
    rs_libp2p_cdr_buffer_read_char(buffer_, &n);
    return *this;
  }
  inline ReadCDRBuffer & operator>>(char16_t & n)
  {
    rs_libp2p_cdr_buffer_read_char16(buffer_, &n);
    return *this;
  }
  inline ReadCDRBuffer & operator>>(float & n)
  {
    rs_libp2p_cdr_buffer_read_float(buffer_, &n);
    return *this;
  }
  inline ReadCDRBuffer & operator>>(double & n)
  {
    rs_libp2p_cdr_buffer_read_double(buffer_, &n);
    return *this;
  }
  inline ReadCDRBuffer & operator>>(bool & n)
  {
    rs_libp2p_cdr_buffer_read_bool(buffer_, &n);
    return *this;
  }
  inline ReadCDRBuffer & operator>>(std::string & s)
  {
    char * data;
    size_t size;
    rs_libp2p_cdr_buffer_read_string(buffer_, &data, &size);
    if (!size) {
      s = std::string();
    } else {
      s = std::string(data, size);
      rs_libp2p_cdr_buffer_free_string(data);
    }
    return *this;
  }

  inline ReadCDRBuffer & operator>>(std::u16string & s)
  {
    char16_t * data;
    size_t size;
    rs_libp2p_cdr_buffer_read_u16string(buffer_, &data, &size);
    s.resize(size);
    for (size_t i = 0; i < size; ++i) {
      *this >> s[i];
    }
    return *this;
  }

private:
  rs_libp2p_cdr_buffer * buffer_;
};
}  // namespace cdr
}  // namespace rmw_libp2p_cpp

#endif  // IMPL__CDR_BUFFER_HPP_
