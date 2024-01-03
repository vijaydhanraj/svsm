// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) Microsoft Corporation
//
// Author: Jon Lange (jlange@microsoft.com)
#pragma once

#include <stdint.h>

typedef enum {
    IGVM_VHT_SUPPORTED_PLATFORM = 0x1,
    IGVM_VHT_PARAMETER_AREA = 0x301,
    IGVM_VHT_PAGE_DATA = 0x302,
    IGVM_VHT_PARAMETER_INSERT = 0x303,
    IGVM_VHT_VP_CONTEXT = 0x304,
    IGVM_VHT_REQUIRED_MEMORY = 0x305,
    IGVM_VHT_VP_COUNT_PARMETER = 0x307,
    IGVM_VHT_MEMORY_MAP = 0x30C,
    IGVM_VHT_ENVIRONMENT_INFO_PARAMETER = 0x313,
} IGVM_VHT;

#define IGVM_MAGIC 0x4D564749

typedef struct {
    uint32_t Magic;
    uint32_t FormatVersion;
    uint32_t VariableHeaderOffset;
    uint32_t VariableHeaderSize;
    uint32_t TotalFileSize;
    uint32_t Checksum;
} IGVM_FIXED_HEADER;

typedef struct {
    uint32_t header_type;
    uint32_t header_size;
} IGVM_VAR_HEADER;

enum {
    IgvmPlatformType_SevSnp = 2,
};

typedef struct {
    uint32_t CompatibilityMask;
    uint8_t HighestVtl;
    uint8_t PlatformType;
    uint16_t PlatformVersion;
    uint64_t SharedGpaBoundary;
} IGVM_VHS_SUPPORTED_PLATFORM;

typedef struct {
    uint64_t NumberOfBytes;
    uint32_t ParameterPageIndex;
    uint32_t FileOffset;
} IGVM_VHS_PARAMETER_AREA;

typedef struct {
    uint64_t GPA;
    uint32_t CompatibilityMask;
    uint32_t FileOffset;
    uint32_t Flags;
    uint16_t DataType;
    uint8_t VtlMask;
    uint8_t Unused;
} IGVM_VHS_PAGE_DATA;

enum {
    IgvmPageType_Normal = 0,
    IgvmPageType_Secrets = 1,
    IgvmPageType_Cpuid = 2,
    IgvmPageType_CpuidExtendedFeatures = 3,
};

typedef struct {
    uint64_t GPA;
    uint32_t CompatibilityMask;
    uint32_t ParameterPageIndex;
} IGVM_VHS_PARAMETER_INSERT;

typedef struct {
    uint32_t ParameterPageIndex;
    uint32_t ByteOffset;
} IGVM_VHS_PARAMETER;

typedef struct {
    uint64_t GPA;
    uint32_t CompatibilityMask;
    uint32_t NumberOfBytes;
    uint32_t Flags;
    uint32_t Reserved;
} IGVM_VHS_REQUIRED_MEMORY;

typedef struct {
    uint64_t GPA;
    uint32_t CompatibilityMask;
    uint32_t FileOffset;
    uint16_t VpIndex;
    uint16_t Reserved;
    uint32_t padding;
} IGVM_VHS_VP_CONTEXT;

typedef struct {
    uint32_t base;
    uint32_t len;
} IgvmParamBlockFwMem;

typedef struct {
    uint32_t start;
    uint32_t size;
    uint32_t _reserved;
    uint32_t secrets_page;
    uint32_t caa_page;
    uint32_t cpuid_page;
    uint32_t reset_addr;
    uint32_t prevalidated_count;
    IgvmParamBlockFwMem prevalidated[8];
} IgvmParamBlockFwInfo;

typedef struct {
    uint32_t param_area_size;
    uint32_t param_page_offset;
    uint32_t memory_map_offset;
    uint32_t cpuid_page;
    uint32_t secrets_page;
    uint16_t debug_serial_port;
    uint16_t _reserved;
    IgvmParamBlockFwInfo firmware;
    uint32_t kernel_reserved_size;
    uint32_t kernel_size;
    uint64_t kernel_base;
} IgvmParamBlock;

