#!/usr/bin/env python3
"""Test basic functionality of the proof-of-response protocol between NEAR nodes."""

import sys
import pathlib
import time
import unittest

sys.path.append(str(pathlib.Path(__file__).resolve().parents[2] / 'lib'))

from utils import LogTracker
from configured_logger import logger
from cluster import init_cluster, start_cluster
from utils import wait_for_blocks

TIMEOUT_SEC = 5

def wait_for_message(node, expected_message, timeout=TIMEOUT_SEC):
    """Wait for the expected message in a node's logs."""
    tracker = LogTracker(node)
    start = time.time()
    while time.time() - start < timeout:
        if tracker.check(expected_message):
            return True
        time.sleep(0.1)
        tracker.reset()
    return False

class ProofOfResponseTest(unittest.TestCase):
    def test_por_basics(self):
        """Test that two nodes can exchange proof-of-response messages."""
        logger.info("Starting proof-of-response basic test")

        nodes = start_cluster(
            num_nodes=2,
            num_observers=0,
            num_shards=1,
            config=None,
            genesis_config_changes=[],
            client_config_changes={
                0: {"network": {"por_enabled": True}},
                1: {"network": {"por_enabled": True}},
            },
        )

        # Wait for nodes to start and produce some blocks 
        wait_for_blocks(nodes[0], count=5, timeout=60)
        logger.info("Nodes started and producing blocks")

        # Check logs for evidence of PoR messages being sent and received
        # The nodes are hardcoded to send "hello" to their peers on startup
        self.assertTrue(wait_for_message(nodes[0], "Sending PoR request"),
            "Node 0 did not send PoR request")
        self.assertTrue(wait_for_message(nodes[1], "Received PoR request"),
            "Node 1 did not receive PoR request")
        self.assertTrue(wait_for_message(nodes[1], "Sending PoR response"),
            "Node 1 did not send PoR response")
        self.assertTrue(wait_for_message(nodes[0], "Received PoR response"),
            "Node 0 did not receive PoR response")

        # Clean up
        for node in nodes:
            node.kill()

        logger.info("Proof-of-response basic test completed successfully")

if __name__ == '__main__':
    unittest.main()