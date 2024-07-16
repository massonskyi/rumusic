"""create table users & role

Revision ID: 3142e105edca
Revises: 
Create Date: 2024-07-16 17:10:31.409206

"""
from datetime import datetime
from typing import Sequence, Union

from alembic import op
import sqlalchemy as sa

# revision identifiers, used by Alembic.
revision: str = '3142e105edca'
down_revision: Union[str, None] = None
branch_labels: Union[str, Sequence[str], None] = None
depends_on: Union[str, Sequence[str], None] = None


def upgrade() -> None:
    # Create roles table
    roles_table = op.create_table(
        'role',
        sa.Column('id', sa.Integer, primary_key=True),
        sa.Column('name', sa.String, nullable=False),
        sa.Column('permissions', sa.JSON),
        sa.PrimaryKeyConstraint('id')
    )
    # Create users table
    users_table = op.create_table(
        'users',
        sa.Column('id', sa.Integer, primary_key=True),
        sa.Column('name', sa.String(length=50), nullable=False),
        sa.Column('email', sa.String(length=320), unique=True, nullable=False),
        sa.Column('username', sa.String(length=50), nullable=False),
        sa.Column('registered_at', sa.TIMESTAMP, default=datetime.utcnow),
        sa.Column('role_id', sa.Integer, sa.ForeignKey('role.id'), nullable=False),
        sa.Column('hashed_password', sa.String, nullable=False),
        sa.Column('is_active', sa.Boolean, default=True, nullable=False),
        sa.Column('is_superuser', sa.Boolean, default=False, nullable=False),
        sa.Column('is_verified', sa.Boolean, default=False, nullable=False),
        sa.Column('storage', sa.String(length=1024)),
    )


def downgrade() -> None:
    op.drop_table('users')
    op.drop_table('role')
