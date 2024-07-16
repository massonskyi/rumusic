# new file
from typing import AsyncGenerator

from sqlalchemy.ext.asyncio import AsyncSession, create_async_engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker
from app.core import DB_HOST, DB_PORT, DB_USER, DB_NAME, DB_PASS

DATABASE_URL = (f"postgresql+asyncpg://"
                f"{DB_USER}:{DB_PASS}@"
                f"{DB_HOST}:{DB_PORT}/{DB_NAME}"
                )

Base = declarative_base()

engine = create_async_engine(DATABASE_URL)

async_session_maker = sessionmaker(engine, class_=AsyncSession, expire_on_commit=False)


