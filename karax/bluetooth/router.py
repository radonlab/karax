from fastapi import APIRouter

router = APIRouter(prefix='/bluetooth')


@router.get('/devices/')
async def get_devices():
    return []
