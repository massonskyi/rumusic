"""
This file contains the configuration of the project. It will be loaded by the main script. It should not be edited
directly.

To change the configuration, edit the file in the same directory as this file. The config file is loaded by the main
script.
"""

import os

__all__ = ['Config', 'load_config']
__version__ = '0.1.0'
__docstring__ = """ 
this is the config module of the project. 
This file is loaded by the main script. 
It should not be edited directly.
"""

from typing import List, Any
from dotenv import load_dotenv


class Config:
    """
    A class that loads the environment variables from the.env file file into the class.
    This class is used to load the environment variables. It is also used to access the environment variables.
    """

    def __init__(self, constants_name: List[str] = None) -> None:
        """
        A class method that loads the environment variables.
        """
        load_dotenv()

        if not constants_name:
            constants_name = ['DB_HOST', 'DB_PORT', 'DB_USER', 'DB_PASS', 'DB_NAME', 'BACKEND_SECRET_COOKIE_KEY']

        for constant_name in constants_name:
            setattr(self, constant_name, None)

    def setup(self) -> 'Config':
        """
        A class method that sets up the environment variables.
        """
        for n, v in vars(self).items():
            if n.startswith('DB_') or n.startswith('BACKEND_'):
                if getattr(self, n) is None:
                    setattr(self, n, os.environ.get(n, None))
                    print(f'{n} =  {v} = {getattr(self, n)}')
        return self

    def getattr(self, n: str) -> Any:
        """
        A method that returns the environment variables. It is used to access the environment variables.
        :param n: The name of the environment variable. It is used to access the environment variables.
        :return: The value of the environment variable. It is used to access the environment variables.
        :raises: AttributeError: If the environment variable is not found. It is used to access the environment variables.
        :raises: KeyError: If the environment variable is not found. It is used to access the environment variables.
        :return: The value of the environment variable. It is used to access the environment variables.
        """
        if not n.startswith('DB_') and not n.startswith('BACKEND_'):
            raise AttributeError(f'{n} is not a valid environment variable.')

        if getattr(self, n) is None:
            raise KeyError(f'{n} is not a valid environment variable.')

        return getattr(self, n)

    def setattr(self, n: str, value: Any) -> None:
        """
        A method that sets the environment variables. It is used to set the environment variables.
        :param n: The name of the environment variable. It is used to set the environment variables.
        :param value: The value of the environment variable. It is used to set the environment variables.
        :raises: AttributeError: If the environment variable is not found. It is used to set the environment variables.
        :raises: KeyError: If the environment variable is not found. It is used to set the environment variables.
        """
        if not n.startswith('DB_') and not n.startswith('BACKEND_'):
            raise AttributeError(f'{n} is not a valid environment variable.')

        if getattr(self, n) is None:
            raise KeyError(f'{n} is not a valid environment variable.')

        setattr(self, n, value)


def load_config(constants_name: List[str] = None) -> object:
    """
    A function that loads the environment variables.
    """
    config = Config(constants_name)
    try:
        config.setup()
    except KeyError as err:
        raise KeyError(f'{err} is not a valid environment variable.')

    except AttributeError as err:
        raise AttributeError(f'{err} is not a valid environment variable.')

    else:    # if there was no exception
        return config

    finally:
        pass

