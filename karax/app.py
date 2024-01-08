"""
Copyright (C) 2024, Radmon.
Use of this source code is governed by the MIT license that can be
found in the LICENSE file.
"""

from fastapi import FastAPI
from .bluetooth.router import router

app = FastAPI()

app.include_router(router)
