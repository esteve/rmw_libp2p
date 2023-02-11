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

#ifndef RMW_LIBP2P_CPP__CDR_BUFFER_HPP_
#define RMW_LIBP2P_CPP__CDR_BUFFER_HPP_

#include "rmw_libp2p_cpp/rmw_libp2p_rs.hpp"

#include <algorithm>
#include <limits>
#include <stdexcept>
#include <string>
#include <vector>

namespace rmw_libp2p_cpp
{
    namespace cdr
    {
        class WriteCDRBuffer
        {
        public:
            explicit WriteCDRBuffer()
            {
                buffer_ = rs_libp2p_cdr_buffer_new();
            }
            ~WriteCDRBuffer()
            {
                rs_libp2p_cdr_buffer_free(buffer_);
            }

            const rs_libp2p_cdr_buffer *data() const noexcept { return buffer_; }

        private:
            rs_libp2p_cdr_buffer *buffer_;
        };

        class ReadCDRBuffer
        {
        public:
            explicit ReadCDRBuffer()
            {
                buffer_ = rs_libp2p_cdr_buffer_new();
            }
            ~ReadCDRBuffer()
            {
                rs_libp2p_cdr_buffer_free(buffer_);
            }
            inline ReadCDRBuffer &operator>>(uint64_t &n)
            {
                rs_libp2p_cdr_buffer_read_uint64(buffer_, &n);
                return *this;
            }
            inline ReadCDRBuffer &operator>>(uint32_t &n)
            {
                rs_libp2p_cdr_buffer_read_uint32(buffer_, &n);
                return *this;
            }
            inline ReadCDRBuffer &operator>>(uint16_t &n)
            {
                rs_libp2p_cdr_buffer_read_uint16(buffer_, &n);
                return *this;
            }
            inline ReadCDRBuffer &operator>>(uint8_t &n)
            {
                rs_libp2p_cdr_buffer_read_uint8(buffer_, &n);
                return *this;
            }
            inline ReadCDRBuffer &operator>>(int64_t &n)
            {
                rs_libp2p_cdr_buffer_read_int64(buffer_, &n);
                return *this;
            }
            inline ReadCDRBuffer &operator>>(int32_t &n)
            {
                rs_libp2p_cdr_buffer_read_int32(buffer_, &n);
                return *this;
            }
            inline ReadCDRBuffer &operator>>(int16_t &n)
            {
                rs_libp2p_cdr_buffer_read_int16(buffer_, &n);
                return *this;
            }
            inline ReadCDRBuffer &operator>>(int8_t &n)
            {
                rs_libp2p_cdr_buffer_read_int8(buffer_, &n);
                return *this;
            }
        private:
            rs_libp2p_cdr_buffer *buffer_;
        };
    } // namespace cdr
} // namespace rmw_libp2p_cpp

#endif // RMW_LIBP2P_CPP__CDR_BUFFER_HPP_
