"""update users

Revision ID: 2a0cb7a23a5f
Revises: 3142e105edca
Create Date: 2024-07-17 11:38:40.633019

"""
from typing import Sequence, Union

from alembic import op
import sqlalchemy as sa


# revision identifiers, used by Alembic.
revision: str = '2a0cb7a23a5f'
down_revision: Union[str, None] = '3142e105edca'
branch_labels: Union[str, Sequence[str], None] = None
depends_on: Union[str, Sequence[str], None] = None


def upgrade() -> None:
    pass


def downgrade() -> None:
    pass
