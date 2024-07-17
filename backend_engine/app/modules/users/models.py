import os
from datetime import datetime
from typing import Optional, Any

from sqlalchemy import (
    Boolean,
    Column,
    ForeignKey,
    Integer,
    String,
    TIMESTAMP,
    JSON, MetaData, Table,
)
from sqlalchemy.orm import relationship, Mapped

from app.db.base import Base
from fastapi_users_db_sqlalchemy import SQLAlchemyBaseUserTable

metadata = MetaData()

role = Table(
    "role",
    metadata,
    Column("id", Integer, primary_key=True),
    Column("name", String, nullable=False),
    Column("permissions", JSON),
)

users = Table(
    "user",
    metadata,
    Column("id", Integer, primary_key=True),
    Column("name", String(length=50), nullable=False),
    Column("email", String(length=320), nullable=False),
    Column("username", String(length=50), nullable=False),
    Column("registered_at", TIMESTAMP, default=datetime.utcnow),
    Column("role_id", Integer, ForeignKey(role.c.id)),
    Column('storage', String(length=1024)),
    Column("hashed_password", String, nullable=False),
    Column("is_active", Boolean, default=True, nullable=False),
    Column("is_superuser", Boolean, default=False, nullable=False),
    Column("is_verified", Boolean, default=False, nullable=False),
)


class User(SQLAlchemyBaseUserTable[int], Base):
    """
    A user of the music player application. A user can have multiple roles and can create,
    modify, and delete tracks and playlists. Each user has a name, email, and a storage path.
    """
    __tablename__: str = "user"

    id: Column = Column(Integer, primary_key=True, autoincrement=True, nullable=False)
    name: Column = Column(String(length=50), nullable=False)
    username: Column = Column(String(length=30), nullable=False)
    registered_at: Column = Column(TIMESTAMP, default=datetime.utcnow())
    role_id: int = Column(Integer, ForeignKey(role.c.id))
    email: Column = Column(
        String(length=320), unique=True, index=True, nullable=False
    )
    hashed_password: Column = Column(String(length=1024), nullable=False)
    is_active: Column = Column(Boolean, default=True, nullable=False)
    is_superuser: Column = Column(Boolean, default=False, nullable=False)
    is_verified: Column = Column(Boolean, default=False, nullable=False)
    storage: str = Column(String)

    tracks = Column(JSON)
    playlists = Column(JSON)

    def __init__(self, *args, **kwargs) -> None:
        """
        Creates a new user. The user's email is used as the username. The user's password is hashed using the bcrypt algorithm.
        If the user is a superuser, the user's password is hashed using the bcrypt algorithm.
        If the user is not a superuser, the user's password is hashed using the bcrypt algorithm.
        creates a storage path for the user. If the storage path already exists, it is deleted. If the storage path does not exist, it is created.
        Args:
            name: str
            username: str
            email: str
            password: str
            is_active: bool
            is_superuser: bool
            is_verified: bool
            storage: str
            tracks: list[Track]
            playlists: list[Playlist]
        Keyword Args:
            tracks: list[Track]
            playlists: list[Playlist]
        :return: None
        """
        super().__init__(*args, **kwargs)
        self.storage = self.create_storage_path()

    def create_storage_path(self) -> str:
        """
        Creates a storage path for the user. If the storage path already exists, it is deleted. If the storage path does not exist, it is created.
        :return: str
        """
        storage_path = os.path.join(
            "storage", f"user_storage_{self.email}", datetime.now().strftime("%Y-%m-%d_%H-%M-%S")
        )
        os.makedirs(storage_path, exist_ok=True)
        return storage_path

    def get_storage_path(self) -> str:
        """
        Returns the storage path of the user.
        :return: str
        """
        return self.storage

    def get_user_id(self) -> int:
        """
        Returns the user's id.
        :return: int
        """
        return self.id

    def get_user_name(self) -> str:
        """
        Returns the user's name.
        :return: str
        """
        return self.name

    def get_user_email(self) -> str:
        """
        Returns the user's email.
        :return: str
        """
        return self.email

    def get_user_username(self) -> str:
        """
        Returns the user's username.
        :return: str
        """
        return self.username

    def get_user_password(self) -> str:
        """
        Returns the user's password.
        :return: str
        """
        return self.hashed_password

    def get_user_unhash_password(self) -> str:
        """
        Returns the user's un hash password.'
        """
        raise NotImplementedError("This function is not implemented yet.")

    def get_user_is_active(self) -> bool:
        """
        Returns the user's is_active.
        :return: bool
        """
        return self.is_active

    def get_user_is_superuser(self) -> bool:
        """
        Returns the user's is_superuser.
        :return: bool
        """
        return self.is_superuser

    def get_user_is_verified(self) -> bool:
        """Returns the user's is_verified."""
        return self.is_verified
