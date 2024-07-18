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
    op.add_column('user', sa.Column('tracks', sa.JSON))
    op.add_column('user', sa.Column('playlists', sa.JSON))


def downgrade() -> None:
    op.drop_table('user')
    op.drop_table('role')
