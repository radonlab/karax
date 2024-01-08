"""
Copyright (C) 2024, Skyler.
Use of this source code is governed by the MIT license that can be
found in the LICENSE file.
"""

from fastapi import APIRouter

router = APIRouter(prefix="/bluetooth")


@router.get("/devices/")
async def get_devices():
    return []
