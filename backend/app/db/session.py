from sqlalchemy.ext.asyncio import AsyncSession
from typing import AsyncGenerator

from app.db.base import async_session_maker


async def get_async_session() -> AsyncGenerator[AsyncSession, None]:
    async with async_session_maker() as session:
        yield session
