from fastapi import Depends
from fastapi_users_db_sqlalchemy import SQLAlchemyUserDatabase
from sqlalchemy.ext.asyncio import AsyncSession

from app.db.session import get_async_session

try:
    from app.modules.users.models import User
except ImportError:
    raise ImportError("User module not found")


async def get_user_db(session: AsyncSession = Depends(get_async_session)) -> SQLAlchemyUserDatabase:
    """
    Get user database instance from SQLAlchemy session dependency.
    :param session: SQLAlchemy session instance. (default: Depends(get_async_session))
    :return: SQLAlchemy user database instance.
    """
    yield SQLAlchemyUserDatabase(session, User)
