"""
Copyright (C) 2024, Skyler.
Use of this source code is governed by the MIT license that can be
found in the LICENSE file.
"""

from fastapi import FastAPI

from .bluetooth import router as bluetooth_router
from .camera import router as camera_router

app = FastAPI()
app.include_router(bluetooth_router)
app.include_router(camera_router)
