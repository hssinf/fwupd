---
title: Plugin: Intel SPI
---

## Introduction

This plugin verifies the SPI contents, typically an Intel Flash descriptor.
The result will be stored in an security attribute for HSI.

## External Interface Access

This plugin requires read access to `/dev/port` and thus will not work if the
kernel is locked down.

## Version Considerations

This plugin has been available since fwupd version `1.6.0`.
